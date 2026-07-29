package com.longbridge.agent;

/**
 * Sensitive content in the user query was masked before processing. Display
 * {@link #getMaskedQuery} instead of the original query.
 */
public final class QueryMaskedEvent extends ConversationStreamEvent {
    private String rawQuery;
    private String maskedQuery;

    /**
     * Returns the original user query.
     *
     * @return original user query
     */
    public String getRawQuery() {
        return rawQuery;
    }

    /**
     * Returns the masked query.
     *
     * @return masked query
     */
    public String getMaskedQuery() {
        return maskedQuery;
    }

    @Override
    public String toString() {
        return "QueryMaskedEvent [rawQuery=" + rawQuery + ", maskedQuery=" + maskedQuery + "]";
    }
}
