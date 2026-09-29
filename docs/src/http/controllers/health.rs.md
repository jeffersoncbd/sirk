## Summary
Returns a JSON response indicating that the service is healthy.

## Behavior
The async function returns HTTP response data containing `HealthResponse` with status `ok`. It performs no validation or fallible operations.

## Imports
- `JsonResponse`: Wraps the health response.
- `HealthResponse`, `SuccessStatus`: Represent the response body and status.
- `ErrorResponse`: Documents the endpoint’s 405 response.
