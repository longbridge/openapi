package com.longbridge.asset;

/**
 * Options for querying statement list
 */
public class GetStatementListOptions {
    /**
     * Statement type: 1 = daily (default), 2 = monthly
     */
    public Integer statementType;

    /**
     * Start date for pagination
     */
    public Integer startDate;

    /**
     * Number of results (default 20)
     */
    public Integer limit;

    public GetStatementListOptions() {
    }

    public GetStatementListOptions setStatementType(int statementType) {
        this.statementType = statementType;
        return this;
    }

    public GetStatementListOptions setStartDate(int startDate) {
        this.startDate = startDate;
        return this;
    }

    public GetStatementListOptions setLimit(int limit) {
        this.limit = limit;
        return this;
    }
}
