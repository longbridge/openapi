package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Overview information for a listed company. */
public class CompanyOverview {
    /** Short name, e.g. {@code "腾讯控股"}. */
    public String name;
    /** Full legal name. */
    public String companyName;
    /** Founding date. */
    public String founded;
    /** Listing date. */
    public String listingDate;
    /** Primary listing market display name. */
    public String market;
    /** Market region code, e.g. {@code "HK"}. */
    public String region;
    /** Registered address. */
    public String address;
    /** Principal office address. */
    public String officeAddress;
    /** Company website. */
    public String website;
    /** IPO issue price. */
    public BigDecimal issuePrice;
    /** Number of shares offered at IPO. */
    public String sharesOffered;
    /** Chairman name. */
    public String chairman;
    /** Company secretary name. */
    public String secretary;
    /** Auditing institution. */
    public String auditInst;
    /** Company classification category. */
    public String category;
    /** Fiscal year end, e.g. {@code "12 月 31 日"}. */
    public String yearEnd;
    /** Number of employees. */
    public String employees;
    /** Phone number. */
    public String phone;
    /** Fax number. */
    public String fax;
    /** Investor relations email. */
    public String email;
    /** Legal representative. */
    public String legalRepr;
    /** CEO / Managing Director. */
    public String manager;
    /** Business licence number. */
    public String busLicense;
    /** Accounting firm. */
    public String accountingFirm;
    /** Securities representative. */
    public String securitiesRep;
    /** Legal counsel. */
    public String legalCounsel;
    /** Postal code. */
    public String zipCode;
    /** Exchange ticker code, e.g. {@code "00700"}. */
    public String ticker;
    /** URL to the company's logo icon. */
    public String icon;
    /** Business profile / description. */
    public String profile;
    /** ADS ratio (may be empty). */
    public String adsRatio;
    /** Industry sector code. */
    public int sector;
}