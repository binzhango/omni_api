class TransformSchemaError(ValueError):
    """Raised when target schema is outside supported MVP subset."""


class TransformValidationError(ValueError):
    """Raised when transformed payload fails required validation checks."""
