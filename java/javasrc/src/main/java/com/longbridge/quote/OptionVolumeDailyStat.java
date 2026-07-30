package com.longbridge.quote;

import java.time.LocalDate;

public class OptionVolumeDailyStat {
    public LocalDate date;
    public long callVolume;
    public long putVolume;
    public long callOpenInterest;
    public long putOpenInterest;
    public double pcVol;
    public double pcOi;
}
