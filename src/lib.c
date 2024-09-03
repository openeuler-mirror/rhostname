#include <stdio.h>
#include <netdb.h>
#include <err.h>

void hostname_alias(const char* name) {
  struct hostent *ent = gethostbyname(name);

  if (ent == NULL) {
		errx(1, "%s", hstrerror(h_errno));
  }

  for (int i = 0; ent->h_aliases[i]; i++) {
    if (i > 0) printf(" ");
    printf("%s", ent->h_aliases[i]);
  }
  printf("\n");
}