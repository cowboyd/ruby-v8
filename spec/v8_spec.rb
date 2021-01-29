# frozen_string_literal: true

RSpec.describe V8 do
  it "can eval a string" do
    expect(V8.eval("'Hello World!'")).to eq("Hello World!")
  end
end
