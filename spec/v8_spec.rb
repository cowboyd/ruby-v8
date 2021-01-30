# frozen_string_literal: true

RSpec.describe V8 do
  it "can eval a string" do
    expect(V8.eval("'Hello World!'")).to eq("Hello World!")
  end
  it "evaluates undefined to nil" do
    expect(V8.eval("undefined")).to eq(nil)
  end
  it "evaluates null to nil" do
    expect(V8.eval("null")).to eq(nil)
  end
  it "evaluates booleans to booleans" do
    expect(V8.eval('true')).to eq(true)
    expect(V8.eval('false')).to eq(false)
  end

  it "evaluates int32 values to integers" do
    expect(V8.eval('-5')).to eq(-5)
  end

  it "evaluates uint32 values to integers" do
    expect(V8.eval('2147483648')).to eq(2147483648)
  end

  it "evaluates int64 values to integers" do
    expect(V8.eval('-2147483650')).to eq(-2147483650)
  end
end
