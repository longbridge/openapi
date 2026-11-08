package com.longbridge.agent;

/**
 * Present when a conversation run failed.
 * <p>
 * This describes a failure of the conversation <em>run itself</em> (the HTTP
 * call succeeded, but the Agent's workflow ended in the {@code failed}
 * status) — it is unrelated to {@link com.longbridge.OpenApiException}, which
 * is thrown for request-level failures (network, auth, malformed request,
 * etc).
 */
public class ConversationError {
    private int code;
    private String message;

    /**
     * Returns the error code.
     *
     * @return error code
     */
    public int getCode() {
        return code;
    }

    /**
     * Returns the error message.
     *
     * @return error message
     */
    public String getMessage() {
        return message;
    }

    @Override
    public String toString() {
        return "ConversationError [code=" + code + ", message=" + message + "]";
    }
}
