use time::Time;

use crate::TradeSession;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FindSessionResult {
    BeforeFirst,
    Between(usize),
    After(usize),
}

pub(crate) trait FindSession {
    fn find_session(&self, time: Time) -> FindSessionResult;
}

impl FindSession for &[TradeSession] {
    fn find_session(&self, time: Time) -> FindSessionResult {
        let trade_sessions = self;

        for (
            idx,
            TradeSession {
                start,
                end,
                inclusive,
                ..
            },
        ) in trade_sessions.iter().enumerate()
        {
            if !*inclusive {
                if time < *start {
                    if idx == 0 {
                        return FindSessionResult::BeforeFirst;
                    } else {
                        return FindSessionResult::After(idx - 1);
                    }
                } else if time < *end {
                    return FindSessionResult::Between(idx);
                }
            } else if time < *start {
                if idx == 0 {
                    return FindSessionResult::BeforeFirst;
                } else {
                    return FindSessionResult::After(idx - 1);
                }
            } else if time <= *end {
                return FindSessionResult::Between(idx);
            }
        }

        FindSessionResult::After(trade_sessions.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::time;

    use super::*;

    #[test]
    fn test_find_session() {
        let sessions: &[TradeSession] = &[
            TradeSession::new(time!(9:30:00), time!(11:30:00)),
            TradeSession::new(time!(13:00:00), time!(15:00:00)),
        ];

        assert_eq!(
            sessions.find_session(time!(9:00:00)),
            FindSessionResult::BeforeFirst
        );
        assert_eq!(
            sessions.find_session(time!(9:29:59)),
            FindSessionResult::BeforeFirst
        );

        assert_eq!(
            sessions.find_session(time!(9:30:00)),
            FindSessionResult::Between(0)
        );
        assert_eq!(
            sessions.find_session(time!(9:30:01)),
            FindSessionResult::Between(0)
        );
        assert_eq!(
            sessions.find_session(time!(11:29:59)),
            FindSessionResult::Between(0)
        );

        assert_eq!(
            sessions.find_session(time!(11:30:00)),
            FindSessionResult::After(0)
        );
        assert_eq!(
            sessions.find_session(time!(12:59:59)),
            FindSessionResult::After(0)
        );

        assert_eq!(
            sessions.find_session(time!(13:00:00)),
            FindSessionResult::Between(1)
        );
        assert_eq!(
            sessions.find_session(time!(14:59:59)),
            FindSessionResult::Between(1)
        );

        assert_eq!(
            sessions.find_session(time!(15:00:00)),
            FindSessionResult::After(1)
        );
        assert_eq!(
            sessions.find_session(time!(16:00:00)),
            FindSessionResult::After(1)
        );
    }
}
