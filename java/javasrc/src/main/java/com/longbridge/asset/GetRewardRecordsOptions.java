package com.longbridge.asset;

/**
 * Options for the reward record list
 */
@SuppressWarnings("unused")
public class GetRewardRecordsOptions {
    private Integer page;
    private Integer pageSize;
    private RewardType rewardType;

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetRewardRecordsOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size (max 50).
     *
     * @param pageSize page size
     * @return this instance for chaining
     */
    public GetRewardRecordsOptions setPageSize(int pageSize) {
        this.pageSize = pageSize;
        return this;
    }

    /**
     * Filters by reward type (default: all types). {@link RewardType#Unknown}
     * is not a server code, so it clears the filter.
     *
     * @param rewardType reward type
     * @return this instance for chaining
     */
    public GetRewardRecordsOptions setRewardType(RewardType rewardType) {
        this.rewardType = rewardType;
        return this;
    }
}
