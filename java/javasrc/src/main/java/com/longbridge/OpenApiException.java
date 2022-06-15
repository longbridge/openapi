package com.longbridge;

public class OpenApiException extends Exception {
    private Long code;
    private String message;

    public OpenApiException(Long code, String message) {
        this.code = code;
        this.message = message;
    }

    public Long getCode() {
        return code;
    }

    public String getMessage() {
        return message;
    }
}
