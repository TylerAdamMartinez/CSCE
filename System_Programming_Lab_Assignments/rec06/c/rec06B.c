#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

int main() {
  char *name;
  pid_t pid;
  pid = fork();

  if (pid == 0) {
    printf("child: %d started\n", getpid());
    printf("child: parent %d\n", getppid());
    printf("child: ...");
    sleep(20);
    printf("child has been awaken!");
  }
  else if (pid > 0) {
    printf("parent: %d started\n", getpid());
    printf("parent: parent %d\n", getppid());
  }
  else {
    perror("fork error\n");
  }

  (pid == 0) ? (name = "child") : (name = "parent");
  printf("%s: terminating... \n", name);
}
