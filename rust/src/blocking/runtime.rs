use std::{sync::Arc, thread};

use futures_util::{future::BoxFuture, Future};
use tokio::sync::mpsc;

use crate::{blocking::BlockingError, Result};

const THREAD_NAME: &str = "longbridge-sync-runtime";

type ExecFn<Ctx> = Box<dyn FnOnce(Arc<Ctx>) -> BoxFuture<'static, ()> + Send + 'static>;

pub(crate) struct BlockingRuntime<Ctx> {
    task_tx: flume::Sender<ExecFn<Ctx>>,
}

impl<Ctx> BlockingRuntime<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub(crate) fn try_new<CreateCtx, CreateCtxFut, PushType, PushCallback>(
        create_ctx: CreateCtx,
        mut push_callback: PushCallback,
    ) -> Result<Self>
    where
        CreateCtx: FnOnce() -> CreateCtxFut + Send + 'static,
        CreateCtxFut: Future<Output = Result<(Ctx, mpsc::UnboundedReceiver<PushType>)>>,
        PushCallback: FnMut(PushType) + Send + 'static,
    {
        let (init_tx, init_rx) = flume::unbounded();
        let (task_tx, task_rx) = flume::unbounded::<ExecFn<Ctx>>();

        // create a thread to execute the future
        thread::Builder::new()
            .name(THREAD_NAME.to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("create tokio runtime");
                let handle = rt.handle().clone();

                rt.block_on(async move {
                    let (ctx, mut event_rx) = match create_ctx().await {
                        Ok(res) => {
                            let _ = init_tx.send(Ok(()));
                            res
                        }
                        Err(err) => {
                            tracing::error!(error = %err, "failed to create quote context");
                            let _ = init_tx.send(Err(err));
                            return;
                        }
                    };
                    let ctx = Arc::new(ctx);

                    loop {
                        tokio::select! {
                            item = task_rx.recv_async() => {
                                match item {
                                    Ok(f) => {
                                        handle.spawn(f(ctx.clone()));
                                    },
                                    Err(_) => break,
                                }
                            }
                            item = event_rx.recv() => {
                                match item {
                                    Some(event) => push_callback(event),
                                    None => break,
                                }
                            }
                        }
                    }
                });
            })
            .expect("spawn thread");

        init_rx
            .recv()
            .expect("recv init event")
            .map(|_| Self { task_tx })
    }

    pub(crate) fn call<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Arc<Ctx>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send,
        R: Send + 'static,
    {
        let (reply_tx, reply_rx) = flume::unbounded();
        self.task_tx
            .send(Box::new(move |ctx| {
                Box::pin(async move {
                    let res = f(ctx).await;
                    let _ = reply_tx.send(res);
                })
            }))
            .map_err(|_| BlockingError::Closed)?;
        reply_rx.recv().map_err(|_| BlockingError::Closed)?
    }
}
