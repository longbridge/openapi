package com.longbridge.quote;

import java.time.OffsetDateTime;

public class QuotePackageDetail {
    private String key;
    private String name;
    private String description;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;

    public String getKey() {
        return key;
    }

    public String getName() {
        return name;
    }

    public String getDescription() {
        return description;
    }

    public OffsetDateTime getStartAt() {
        return startAt;
    }

    public OffsetDateTime getEndAt() {
        return endAt;
    }

    @Override
    public String toString() {
        return "QuotePackageDetail [key=" + key + ", name=" + name + ", description=" + description + ", startAt="
                + startAt + ", endAt=" + endAt + "]";
    }
}