package com.longbridge.trade;

import java.lang.reflect.Type;
import java.util.Arrays;
import java.util.Collections;
import java.util.Map;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;

/**
 * Risk assessment (suitability questionnaire) result of the current member
 */
public class RiskAssessment {
    private static final Gson GSON = new Gson();
    private static final Type STRING_MAP = new TypeToken<Map<String, String>>() {
    }.getType();
    private static final Type LONG_MAP = new TypeToken<Map<String, Long>>() {
    }.getType();

    private String risk;
    private int pi;
    // JSON object text, parsed on access
    private String approvals;
    private String reAuthentication;
    private long piEndAt;
    // JSON object text, parsed on access
    private String deadlines;
    private String riskAssessmentAt;
    private int score;
    private String[] strategyRiskLevels;
    private transient Map<String, String> approvalsMap;
    private transient Map<String, Long> deadlinesMap;

    /**
     * Returns the risk level of the latest questionnaire (e.g. {@code A5}).
     *
     * <p>The server does not document the scale, so do not infer an ordering
     * between levels.
     *
     * @return risk level
     */
    public String getRisk() {
        return risk;
    }

    /**
     * Returns the professional investor (PI) status code. The server does not
     * document the code values; treat it as opaque.
     *
     * @return PI status code
     */
    public int getPi() {
        return pi;
    }

    /**
     * Returns the approval states keyed by item (e.g. {@code car}, {@code cka},
     * {@code risk_audit}).
     *
     * <p>Values are strings, not booleans: {@code is_corporate_client} arrives
     * as {@code "true"} / {@code "false"}.
     *
     * @return approval states, in key order (read-only); empty when the server
     *         returns none
     */
    public synchronized Map<String, String> getApprovals() {
        if (approvalsMap == null) {
            approvalsMap = parseMap(approvals, STRING_MAP);
        }
        return approvalsMap;
    }

    /**
     * Returns the re-authentication flag, as returned by the server (empty when
     * not set).
     *
     * @return re-authentication flag
     */
    public String getReAuthentication() {
        return reAuthentication;
    }

    /**
     * Returns the professional investor status end time, as returned by the
     * server ({@code 0} when not set). Raw integer: the unit (seconds or
     * milliseconds) is not documented, so it is not converted to a timestamp.
     *
     * @return PI end time
     */
    public long getPiEndAt() {
        return piEndAt;
    }

    /**
     * Returns the deadlines keyed by item, as returned by the server.
     *
     * <p>Raw integers with the same undocumented unit as {@code piEndAt}.
     *
     * @return deadlines, in key order (read-only); empty when the server returns
     *         none
     */
    public synchronized Map<String, Long> getDeadlines() {
        if (deadlinesMap == null) {
            deadlinesMap = parseMap(deadlines, LONG_MAP);
        }
        return deadlinesMap;
    }

    /**
     * Returns the date of the latest risk assessment, as returned by the server
     * (e.g. {@code 2026-10-14}).
     *
     * <p>Kept as the server's string: the {@code yyyy-mm-dd} form is not
     * documented to be guaranteed. May be empty; a null or absent value also
     * decodes to an empty string, so emptiness does not by itself mean
     * "never assessed".
     *
     * @return risk assessment date
     */
    public String getRiskAssessmentAt() {
        return riskAssessmentAt;
    }

    /**
     * Returns the questionnaire score.
     *
     * @return questionnaire score
     */
    public int getScore() {
        return score;
    }

    /**
     * Returns the strategy risk levels the member is allowed to trade.
     *
     * @return strategy risk levels
     */
    public String[] getStrategyRiskLevels() {
        return strategyRiskLevels;
    }

    private static <V> Map<String, V> parseMap(String json, Type type) {
        if (json == null || json.isEmpty()) {
            return Collections.emptyMap();
        }
        Map<String, V> map = GSON.fromJson(json, type);
        return map == null ? Collections.<String, V>emptyMap() : Collections.unmodifiableMap(map);
    }

    @Override
    public String toString() {
        return "RiskAssessment [risk=" + risk + ", pi=" + pi + ", approvals=" + getApprovals()
                + ", reAuthentication=" + reAuthentication + ", piEndAt=" + piEndAt + ", deadlines="
                + getDeadlines() + ", riskAssessmentAt=" + riskAssessmentAt + ", score=" + score
                + ", strategyRiskLevels=" + Arrays.toString(strategyRiskLevels) + "]";
    }

}
