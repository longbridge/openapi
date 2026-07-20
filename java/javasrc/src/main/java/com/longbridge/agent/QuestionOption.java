package com.longbridge.agent;

/**
 * One option of a {@link Question}
 */
public class QuestionOption {
    private String description;

    /**
     * Returns the option text.
     *
     * @return option text
     */
    public String getDescription() {
        return description;
    }

    @Override
    public String toString() {
        return "QuestionOption [description=" + description + "]";
    }
}
