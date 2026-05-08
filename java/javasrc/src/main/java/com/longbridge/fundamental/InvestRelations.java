package com.longbridge.fundamental;

/** Securities in which the queried company holds a stake. */
public class InvestRelations {
    /** Link to the full investor-relations page. */
    public String forwardUrl;
    /** Securities in which the queried company holds an investment stake. */
    public InvestSecurity[] investSecurities;
}