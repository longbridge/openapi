package com.longbridge.agent;

/**
 * One option of a {@link Question}
 */
public class QuestionOption {
    private String label;
    private String description;

    /**
     * Returns the short UI label for the option.
     *
     * @return label
     */
    public String getLabel() {
        return label;
    }

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
        return "QuestionOption [label=" + label + ", description=" + description + "]";
    }
}
