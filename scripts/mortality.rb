require "csv"
require "pp"

data = Hash.new { |hash, key| hash[key] = [0]*15 }

CSV.foreach("data/extract1.csv", headers: true) do |row|
  data[row["Cause"]][row["Year"].to_i - 1999] += row["All ages"].to_i
end

pp data.to_a.sample(100)
