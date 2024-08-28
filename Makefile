NAME := target/debug/computor

$(NAME):
	cargo build

all: $(NAME)

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re
