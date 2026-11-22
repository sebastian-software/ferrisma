/* Neutral <errno.h>: the error constants libaom compares with are the same
 * on Darwin and Linux; errno itself goes through crate::platform. */
#ifndef FERRISMA_NEUTRAL_ERRNO_H_
#define FERRISMA_NEUTRAL_ERRNO_H_
int *ferrisma_errno(void);
#define errno (*ferrisma_errno())
#define EINVAL 22
#define ERANGE 34
#define ENOMEM 12
#endif
