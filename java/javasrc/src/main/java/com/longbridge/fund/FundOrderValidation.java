package com.longbridge.fund;

/**
 * FundOrderValidation
 */
public class FundOrderValidation {
    private String authToken;
    private String evalAddress;
    private int fundRiskLevel;
    private String msg;
    private int userPi;
    private int userRiskLevel;

    /**
     * Returns authToken.
     *
     * @return authToken
     */
    public String getAuthToken() {
        return authToken;
    }

    /**
     * Returns evalAddress.
     *
     * @return evalAddress
     */
    public String getEvalAddress() {
        return evalAddress;
    }

    /**
     * Returns fundRiskLevel.
     *
     * @return fundRiskLevel
     */
    public int getFundRiskLevel() {
        return fundRiskLevel;
    }

    /**
     * Returns msg.
     *
     * @return msg
     */
    public String getMsg() {
        return msg;
    }

    /**
     * Returns userPi.
     *
     * @return userPi
     */
    public int getUserPi() {
        return userPi;
    }

    /**
     * Returns userRiskLevel.
     *
     * @return userRiskLevel
     */
    public int getUserRiskLevel() {
        return userRiskLevel;
    }

    @Override
    public String toString() {
        return                 "FundOrderValidation [authToken=" + authToken +
                ", evalAddress=" + evalAddress +
                ", fundRiskLevel=" + fundRiskLevel +
                ", msg=" + msg +
                ", userPi=" + userPi +
                ", userRiskLevel=" + userRiskLevel + "]";
    }
}
