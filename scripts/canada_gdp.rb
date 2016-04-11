require "csv"
require "pp"

data = Hash.new { |hash, key| hash[key] = []}

CSV.foreach("data/canada-gdp-by-industry.csv", headers: true) do |row|
  name = "Canadian GDP from #{row["North American Industry Classification System (NAICS)"].downcase.split('[')[0][0..-2]}"
  (1997..2016).each do |year|
    Date::ABBR_MONTHNAMES.each_with_index do |month_name, month_number|
      month_key = "#{month_name}-#{year}"
      if row.key?(month_key) and row[month_key] != ".." then
        data[name].push([Time.new(year, month_number).to_i, row[month_key].to_i])
      end
    end
  end
end

# ==== Write out file

f = File.open("btsf/canada_gdp.btsf","w")

# Header
f.print [1, 4*4, 2*4, data.size].pack("L*")

# Data
data.each do |name, points|
  f.print [points.length, name.length].pack("L*")
  f.print name
  points.each do |d,v|
    f.print [d, v].pack("le")
  end
end
