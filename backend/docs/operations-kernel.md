# Operations Kernel

All meaningful business lifecycle changes must pass through the operations kernel.

No handler, frontend PATCH route, Stripe webhook, or admin action should directly mutate engagement status.

The Kernel is responsible for:

1. validating the current state
2. applying the allowed transition
3. enforcing payment/contract/milestone rules
4. recording an operational event
5. dispatching notifications later
