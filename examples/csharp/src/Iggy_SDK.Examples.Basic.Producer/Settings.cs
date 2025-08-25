// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

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
