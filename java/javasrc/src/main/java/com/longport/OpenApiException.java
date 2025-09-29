package com.longport;

public class OpenApiException extends Exception {
    private ErrorKind kind;
    private Long code;
    private String message;

    public OpenApiException(ErrorKind kind, Long code, String message) {
        this.kind = kind;
        this.code = code;
        this.message = message;
    }

    public ErrorKind getKind() {
        return kind;
    }

    public Long getCode() {
        return code;
    }

    public String getMessage() {
        return message;
    }

    @Override
    public String toString() {
        return "OpenApiException [kind=" + kind + ", code=" + code + ", message=" + message + "]";
    }
}
