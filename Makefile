.PHONY: build deploy
build:
	spin build

deploy:
	spin build && spin deploy
