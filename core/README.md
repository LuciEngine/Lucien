# Lucien Core

Contains middlewares that you may use anywhere for the application.

Middleware: usually created as singletons here so you don't have to pass in from the very top of application to the bottom. For example, you don't want a logger passed down from application entrypoint to the lowest level of your graphics API. Instead, you want _direct_ access globally. This is what the package is designed for.
