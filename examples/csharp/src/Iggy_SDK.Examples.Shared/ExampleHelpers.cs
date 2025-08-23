using System.Text;
using Apache.Iggy;
using Apache.Iggy.Contracts.Http;
using Apache.Iggy.Exceptions;
using Apache.Iggy.IggyClient;
using Apache.Iggy.Messages;

namespace Iggy_SDK.Examples.Shared;

public static class ExampleHelpers
{
    public static async Task EnsureStreamExists(IIggyClient client, Identifier streamId, string streamName, CancellationToken token = default)
    {
        try
        {
            await client.GetStreamByIdAsync(streamId, token);
        }
        catch (InvalidResponseException)
        {
            await client.CreateStreamAsync(streamName, token: token);
        }
    }
    
    public static async Task EnsureTopicExists(IIggyClient client,
        Identifier streamId,
        Identifier topicId,
        string topicName,
        uint partitionsCount,
        CancellationToken cancellationToken = default
        )
    {
        try
        {
            await client.GetTopicByIdAsync(streamId, topicId, cancellationToken);
        }
        catch (InvalidResponseException)
        {
            await client.CreateTopicAsync(streamId, topicName, partitionsCount, token: cancellationToken);
        }
    }

    public static Message CreateMessage(int messageId)
    {
        return new Message(Guid.NewGuid(),  Encoding.UTF8.GetBytes($"message-{messageId}"));
    }
}