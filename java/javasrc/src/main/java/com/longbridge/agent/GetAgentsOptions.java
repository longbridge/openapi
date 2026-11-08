package com.longbridge.agent;

/**
 * Options for {@link AgentContext#agents}
 */
@SuppressWarnings("unused")
public class GetAgentsOptions {
    private Integer page;
    private Integer limit;
    private String name;

    /**
     * Sets the page number, starts at 1.
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetAgentsOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size.
     *
     * @param limit page size
     * @return this instance for chaining
     */
    public GetAgentsOptions setLimit(Integer limit) {
        this.limit = limit;
        return this;
    }

    /**
     * Fuzzy search by Agent name.
     *
     * @param name Agent name
     * @return this instance for chaining
     */
    public GetAgentsOptions setName(String name) {
        this.name = name;
        return this;
    }
}
