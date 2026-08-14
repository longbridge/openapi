package com.longbridge.agent;

import java.util.Arrays;

/**
 * A message within a chat
 */
public class ChatMessage {
    private long id;
    private long chatId;
    private String chatUid;
    private long agentId;
    private String agentName;
    private String agentUid;
    private String sender;
    private int status;
    private int likes;
    private long parentMessageId;
    private int thinkingSeconds;
    private int errorCode;
    private String workflowRunId;
    private long createdAt;
    private long updatedAt;
    private ChatMessageChunk[] chunks;
    private String extendsData;

    /**
     * Returns the message ID.
     *
     * @return message ID
     */
    public long getId() {
        return id;
    }

    /**
     * Returns the ID of the owning chat.
     *
     * @return chat ID
     */
    public long getChatId() {
        return chatId;
    }

    /**
     * Returns the UID of the owning chat.
     *
     * @return chat UID
     */
    public String getChatUid() {
        return chatUid;
    }

    /**
     * Returns the ID of the Agent.
     *
     * @return Agent ID
     */
    public long getAgentId() {
        return agentId;
    }

    /**
     * Returns the name of the Agent.
     *
     * @return Agent name
     */
    public String getAgentName() {
        return agentName;
    }

    /**
     * Returns the UID of the Agent.
     *
     * @return Agent UID
     */
    public String getAgentUid() {
        return agentUid;
    }

    /**
     * Returns the sender, e.g. {@code user} or {@code assistant}.
     *
     * @return sender
     */
    public String getSender() {
        return sender;
    }

    /**
     * Returns the message status.
     *
     * @return message status
     */
    public int getStatus() {
        return status;
    }

    /**
     * Returns the number of likes.
     *
     * @return number of likes
     */
    public int getLikes() {
        return likes;
    }

    /**
     * Returns the ID of the parent message; 0 if none.
     *
     * @return parent message ID
     */
    public long getParentMessageId() {
        return parentMessageId;
    }

    /**
     * Returns the thinking time in seconds.
     *
     * @return thinking seconds
     */
    public int getThinkingSeconds() {
        return thinkingSeconds;
    }

    /**
     * Returns the error code; 0 if none.
     *
     * @return error code
     */
    public int getErrorCode() {
        return errorCode;
    }

    /**
     * Returns the workflow run ID.
     *
     * @return workflow run ID
     */
    public String getWorkflowRunId() {
        return workflowRunId;
    }

    /**
     * Returns the creation time, Unix timestamp in seconds.
     *
     * @return creation time
     */
    public long getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns the last updated time, Unix timestamp in seconds.
     *
     * @return last updated time
     */
    public long getUpdatedAt() {
        return updatedAt;
    }

    /**
     * Returns the content chunks of the message.
     *
     * @return content chunks
     */
    public ChatMessageChunk[] getChunks() {
        return chunks;
    }

    /**
     * Returns the extension payload (wire field {@code extends}), as a raw JSON
     * string.
     *
     * @return extension payload JSON
     */
    public String getExtendsData() {
        return extendsData;
    }

    @Override
    public String toString() {
        return "ChatMessage [id=" + id + ", chatId=" + chatId + ", chatUid=" + chatUid + ", agentId="
                + agentId + ", agentName=" + agentName + ", agentUid=" + agentUid + ", sender=" + sender
                + ", status=" + status + ", likes=" + likes + ", parentMessageId=" + parentMessageId
                + ", thinkingSeconds=" + thinkingSeconds + ", errorCode=" + errorCode + ", workflowRunId="
                + workflowRunId + ", createdAt=" + createdAt + ", updatedAt=" + updatedAt + ", chunks="
                + Arrays.toString(chunks) + ", extendsData=" + extendsData + "]";
    }
}
