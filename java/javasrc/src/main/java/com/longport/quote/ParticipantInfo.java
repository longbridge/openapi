package com.longport.quote;

import java.util.Arrays;

public class ParticipantInfo {
    private int[] brokerIds;
    private String nameCn;
    private String nameEn;
    private String nameHk;

    public int[] getBrokerIds() {
        return brokerIds;
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
        return "ParticipantInfo [brokerIds=" + Arrays.toString(brokerIds) + ", nameCn=" + nameCn + ", nameEn=" + nameEn
                + ", nameHk=" + nameHk + "]";
    }
}
