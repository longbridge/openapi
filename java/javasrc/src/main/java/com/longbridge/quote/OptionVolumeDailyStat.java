package com.longbridge.quote;

import java.time.LocalDate;

public class OptionVolumeDailyStat {
    public String symbol;
    public LocalDate date;
    public long callVolume;
    public long putVolume;
    public long callOpenInterest;
    public long putOpenInterest;
    public long totalVolume;
    public long totalOpenInterest;
    public double pcVol;
    public double pcOi;
}
