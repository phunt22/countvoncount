description:
count von count, adding custom tools to openai models to make it better at counting. 

tools:
- calculator
- date/time tool

setup:
- clone
- .env <- OPENAI_API_KEY=<here>
- cargo build --release
-- cargo install --path .
-- cvc <prompt>
-- cvc --combine
