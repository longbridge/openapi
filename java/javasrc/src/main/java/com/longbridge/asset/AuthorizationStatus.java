package com.longbridge.asset;

/**
 * Status of a bank-card authorization or of an eDDA application
 */
public enum AuthorizationStatus {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Not authorized */
    Unauthorized,
    /** Authorized */
    Authorized,
    /** Authorization in progress */
    Authorizing,
    /** Authorization failed */
    Failed,
}
