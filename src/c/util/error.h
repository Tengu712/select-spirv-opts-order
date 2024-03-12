#pragma once

#ifndef RELEASE_BUILD
# define ERROR_IF(condExpr, funName, message, destruction, retValue) \
    if ((condExpr)) {                                               \
        printf("[ error ] %s: %s\n", (funName), (message));         \
        destruction;                                                \
        return (retValue);                                          \
    }
#else
# define ERROR_IF(condExpr, funName, message, destruction, retValue) \
    if ((condExpr)) {                                               \
        destruction;                                                \
        return (retValue);                                          \
    }
#endif
