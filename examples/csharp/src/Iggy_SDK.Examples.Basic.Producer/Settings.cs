using Apache.Iggy.Enums;

namespace Iggy_SDK.Examples.Basic.Producer;

public sealed class Settings
{
    public Protocol Protocol { get; set; } = Protocol.Tcp;
    public string BaseAddress { get; set; } = "127.0.0.1:8090";
    public string Username { get; set; } = "iggy";
    public string Password { get; set; } = "iggy";
    public string StreamName { get; set; } = "example-basic";
    public string TopicName { get; set; } = "example-basic-topic";
    public int MessageBatchesLimit { get; set; } = 100;
    public int MessagesPerBatch { get; set; } = 10;
    public TimeSpan Interval { get; set; } = TimeSpan.FromSeconds(1);
    public uint PartitionsCount { get; set; } = 1;
}