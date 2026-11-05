package com.longbridge;

/**
 * Exception thrown by the Longbridge OpenAPI SDK.
 * <p>
 * Every SDK operation that can fail will throw this exception. It carries three
 * pieces of diagnostic information:
 * <ul>
 *   <li>{@link #getKind()} – broad category of the error (HTTP, OpenAPI, OAuth, Other)</li>
 *   <li>{@link #getCode()} – numeric error code: the HTTP status or OpenAPI code returned
 *       by the server, a stable OAuth error code for {@link ErrorKind#OAuth} (see
 *       {@link #getCode()}), or {@code null} for other client-side errors</li>
 *   <li>{@link #getMessage()} – human-readable error description</li>
 * </ul>
 */
public class OpenApiException extends Exception {
    private ErrorKind kind;
    private Long code;
    private String message;

    /**
     * Constructs an {@code OpenApiException}.
     *
     * @param kind    error kind
     * @param code    numeric error code (may be {@code null})
     * @param message human-readable error description
     */
    public OpenApiException(ErrorKind kind, Long code, String message) {
        this.kind = kind;
        this.code = code;
        this.message = message;
    }

    /**
     * Returns the error kind.
     *
     * @return error kind
     */
    public ErrorKind getKind() {
        return kind;
    }

    /**
     * Returns the numeric error code.
     *
     * <p>
     * For {@link ErrorKind#OAuth} errors this is a stable code identifying the
     * failure: 1 no token available, 2 no refresh token, 3 refresh failed, 4 revoke failed, 5 authorization failed, 6 authorization timeout, 7 code exchange failed, 8 CSRF mismatch, 9 callback bind failed, 10 local address failed, 11 callback channel closed, 12 no home dir, 13 token file read, 14 token file parse, 15 token file write, 16 token file delete, 17 create dir failed, 18 serialize token, 0 other.
     *
     * @return error code, or {@code null} for client-side errors other than OAuth
     */
    public Long getCode() {
        return code;
    }

    /**
     * Returns the human-readable error description.
     *
     * @return error message
     */
    public String getMessage() {
        return message;
    }

    @Override
    public String toString() {
        return "OpenApiException [kind=" + kind + ", code=" + code + ", message=" + message + "]";
    }
}
