require "csv"
require "pp"
require "json"

# Death codes
codes = {}

IO.foreach("data/icd10cm_order_2016.txt") do |line|
  code = line[6..13].rstrip
  next unless code.length == 4 || code.length == 3
  desc = line[77..-1].rstrip
  # p [code, desc]
  codes[code] = desc
end

# Mortality data
data = Hash.new { |hash, key| hash[key] = [0]*15 }

def add_data(codes, data, cause, year, count)
  cause = codes[cause]
  return if cause == nil
  name = "Deaths by #{cause[0].downcase}#{cause[1..-1]}"
  data[name][year - 1999] += count
end

CSV.foreach("data/extract1.csv", headers: true) do |row|
  add_data(codes, data, row["Cause"], row["Year"].to_i, row["All ages"].to_i)
  add_data(codes, data, row["Cause"][0..-2], row["Year"].to_i, row["All ages"].to_i)
end

pp data.to_a.sample(100)
p data.size

# puts({"name" => data.to_a[0][0],
#   "data" => data.to_a[0][1].map.with_index {|d,i| {t: Time.new(1999+i).to_i, v: d}},
#   }.to_json)

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
