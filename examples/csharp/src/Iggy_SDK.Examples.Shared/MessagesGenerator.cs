using System.Diagnostics;

namespace Iggy_SDK.Examples.Shared;

public class MessagesGenerator
{
    private ulong _orderId;

    private static string[] CurrencyPairs => ["EUR/USD", "EUR/GBP", "USD/GBP", "EUR/PLN", "USD/PLN"];

    public ISerializableMessage Generate()
    {
        return (Random.Shared.Next() % 3) switch
        {
            0 => GenerateOrderCreated(),
            1 => GenerateOrderConfirmed(),
            2 => GenerateOrderRejected(),
            _ => throw new UnreachableException()
        };
    }

    private OrderCreated GenerateOrderCreated()
    {
        _orderId++;
        return new OrderCreated(
            _orderId,
            CurrencyPairs[Random.Shared.Next(0, CurrencyPairs.Length)],
            Random.Shared.NextDouble() * 990.0 + 10.0,
            Random.Shared.NextDouble() * 0.9 + 0.1,
            Random.Shared.Next() % 2 == 1 ? "buy" : "sell",
            DateTimeOffset.UtcNow
        );
    }

    private OrderConfirmed GenerateOrderConfirmed()
    {
        return new OrderConfirmed(
            _orderId,
            Random.Shared.NextDouble() * 990.0 + 10.0,
            DateTimeOffset.UtcNow
        );
    }

    private OrderRejected GenerateOrderRejected()
    {
        return new OrderRejected(
            _orderId,
            DateTimeOffset.UtcNow,
            Random.Shared.Next() % 2 == 1 ? "cancelled_by_user" : "other"
        );
    }
}