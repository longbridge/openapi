package com.longbridge.grid;

/**
 * Action taken when a grid boundary is reached.
 */
public enum GridLimitEvent {
    /** Unknown / unset */
    Unknown,
    /** Ignore &mdash; keep the grid running */
    Ignore,
    /** Close the position at the last price */
    CloseAtLast,
}
