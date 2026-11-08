package com.longbridge.agent;

import java.util.Arrays;

/**
 * Response for {@link AgentContext#agents}
 */
public class AgentsResponse {
    private Agent[] agents;
    private int total;

    /**
     * Returns the Agent list.
     *
     * @return Agent list
     */
    public Agent[] getAgents() {
        return agents;
    }

    /**
     * Returns the total number of matching Agents.
     *
     * @return total number of matching Agents
     */
    public int getTotal() {
        return total;
    }

    @Override
    public String toString() {
        return "AgentsResponse [agents=" + Arrays.toString(agents) + ", total=" + total + "]";
    }
}
