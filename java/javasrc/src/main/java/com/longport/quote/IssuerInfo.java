package com.longport.quote;

public class IssuerInfo {
    private int issuerId;
    private String nameCn;
    private String nameEn;
    private String nameHk;

    public int getIssuerId() {
        return issuerId;
    }

    public String getNameCn() {
        return nameCn;
    }

    public String getNameEn() {
        return nameEn;
    }

    public String getNameHk() {
        return nameHk;
    }

    @Override
    public String toString() {
        return "IssuerInfo [issuerId=" + issuerId + ", nameCn=" + nameCn + ", nameEn=" + nameEn + ", nameHk=" + nameHk
                + "]";
    }
}
