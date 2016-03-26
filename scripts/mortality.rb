require "csv"
require "pp"

# Death codes
codes = {}

IO.foreach("data/icd10cm_order_2016.txt") do |line|
  code = line[6..13].rstrip
  next unless code.length == 4
  desc = line[77..-1].rstrip
  # p [code, desc]
  codes[code] = desc
end

# Mortality data
data = Hash.new { |hash, key| hash[key] = [0]*15 }

unknown_cause = 0
CSV.foreach("data/extract1.csv", headers: true) do |row|
  cause = row["Cause"]
  cause = codes[cause]
  if cause == nil
    unknown_cause += 1
    next
  end
  data[cause][row["Year"].to_i - 1999] += row["All ages"].to_i
end

p unknown_cause

pp data.to_a.sample(100)

# ==== Write out file

f = File.open("btsf/mortality.btsf","w")

# Header
f.print [1, 4*4, 2*4, data.size].pack("L*")

# Data
data.each do |name, points|
  f.print [15, name.length].pack("L*")
  f.print name
  points.each_with_index do |d,i|
    f.print [Time.new(1999+i).to_i, d].pack("le")
  end
end
