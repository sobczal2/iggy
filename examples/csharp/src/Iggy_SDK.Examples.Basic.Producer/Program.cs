using Apache.Iggy;
using Apache.Iggy.Contracts;
using Apache.Iggy.Factory;
using Apache.Iggy.Kinds;
using Apache.Iggy.Messages;
using Iggy_SDK.Examples.Basic.Producer;
using Iggy_SDK.Examples.Shared;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Logging;

var loggerFactory = LoggerFactory.Create(b => { b.AddConsole(); });
var logger = loggerFactory.CreateLogger<Program>();;
var configuration = new ConfigurationBuilder()
    .AddJsonFile("appsettings.json")
    .Build();
var settings = configuration.Get<Settings>() ?? new Settings();

logger.LogInformation("Basic producer has started, selected protocol: {Protocol}", settings.Protocol);

var client = MessageStreamFactory.CreateMessageStream(opt =>
{
    opt.BaseAdress = settings.BaseAddress;
    opt.Protocol = settings.Protocol;
}, loggerFactory);

await client.LoginUser(settings.Username, settings.Password);

logger.LogInformation("Basic producer has logged on successfully");

var streamId = Identifier.String(settings.StreamName);
var topicId = Identifier.String(settings.TopicName);

await ExampleHelpers.EnsureStreamExists(client, streamId, settings.StreamName);
await ExampleHelpers.EnsureTopicExists(client, streamId, topicId, settings.TopicName, settings.PartitionsCount);

var sentBatches = 0;
var currentId = 0;
while (true)
{
    if (settings.MessageBatchesLimit > 0 && sentBatches == settings.MessageBatchesLimit)
    {
        logger.LogInformation("Sent {SentBatches} batches of messages, exiting.", sentBatches);
        break;
    }
    
    var messages = Enumerable.Range(currentId, currentId + settings.MessagesPerBatch)
        .Aggregate(new List<Message>(settings.MessagesPerBatch), (list, next) =>
        {
             list.Add(ExampleHelpers.CreateMessage(next));
             return list;
        });
    
    await client.SendMessagesAsync(new MessageSendRequest
    {
        StreamId = streamId,
        TopicId = topicId,
        Messages = messages,
        Partitioning = Partitioning.None()
    });

    logger.LogInformation($"Sent messages from id: {currentId} to: {currentId + settings.MessagesPerBatch}");
    currentId += settings.MessagesPerBatch;
    sentBatches++;
    
    await Task.Delay(settings.Interval);
}