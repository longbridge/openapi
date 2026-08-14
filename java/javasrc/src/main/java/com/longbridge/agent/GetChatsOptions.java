package com.longbridge.agent;

/**
 * Options for {@link AgentContext#chats}
 */
@SuppressWarnings("unused")
public class GetChatsOptions {
    private Integer page;
    private Integer limit;
    private String excludeAgentUids;

    /**
     * Sets the page number, starts at 1.
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetChatsOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size.
     *
     * @param limit page size
     * @return this instance for chaining
     */
    public GetChatsOptions setLimit(Integer limit) {
        this.limit = limit;
        return this;
    }

    /**
     * Excludes chats belonging to the given Agent UIDs (comma-joined, e.g.
     * {@code dsl_builder}).
     *
     * @param excludeAgentUids comma-joined Agent UIDs to exclude
     * @return this instance for chaining
     */
    public GetChatsOptions setExcludeAgentUids(String excludeAgentUids) {
        this.excludeAgentUids = excludeAgentUids;
        return this;
    }
}
