package com.longbridge.agent;

import java.util.Arrays;

/**
 * Response for {@link AgentContext#workspaces}
 */
public class WorkspacesResponse {
    private Workspace[] workspaces;

    /**
     * Returns the Workspaces the current account belongs to.
     *
     * @return Workspace list
     */
    public Workspace[] getWorkspaces() {
        return workspaces;
    }

    @Override
    public String toString() {
        return "WorkspacesResponse [workspaces=" + Arrays.toString(workspaces) + "]";
    }
}
