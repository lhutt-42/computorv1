NAME := target/debug/computorv1

$(NAME):
	cargo build

all: $(NAME)

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re
