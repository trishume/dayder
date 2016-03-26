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

CSV.foreach("data/extract1.csv", headers: true) do |row|
  cause = row["Cause"]
  cause = codes[cause]
  data[cause][row["Year"].to_i - 1999] += row["All ages"].to_i
end

pp data.to_a.sample(100)
