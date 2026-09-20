package com.longbridge.fundamental;

/**
 * Financial statement kind.
 *
 * <p>
 * Unlike {@link FinancialReportKind} there is no {@code All}: the statements
 * endpoint needs one specific statement per request and answers with an empty
 * list for {@code ALL}.
 */
public enum FinancialStatementKind {
    /** Income statement (IS) */
    IncomeStatement,
    /** Balance sheet (BS) */
    BalanceSheet,
    /** Cash flow statement (CF) */
    CashFlow,
}
