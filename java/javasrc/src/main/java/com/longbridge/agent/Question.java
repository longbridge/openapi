package com.longbridge.agent;

import java.util.Arrays;

/**
 * One question the Agent needs you to answer
 */
public class Question {
    private String question;
    private QuestionOption[] options;
    private boolean multiSelect;

    /**
     * Returns the question text.
     *
     * @return question text
     */
    public String getQuestion() {
        return question;
    }

    /**
     * Returns the options; empty means free-form answer.
     *
     * @return options
     */
    public QuestionOption[] getOptions() {
        return options;
    }

    /**
     * Returns whether multiple options may be selected.
     *
     * @return {@code true} if multiple options may be selected
     */
    public boolean isMultiSelect() {
        return multiSelect;
    }

    @Override
    public String toString() {
        return "Question [question=" + question + ", options=" + Arrays.toString(options) + ", multiSelect="
                + multiSelect + "]";
    }
}
