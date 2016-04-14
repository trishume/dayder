require "csv"
require "time"

def get_points(file)
  pts = []
  CSV.foreach(file, headers: true) do |row|
    time = Time.parse(row["DATE".freeze])
    value = row["VALUE"]
    next if value == '.'.freeze
    pts << [time.to_i, value.to_f]
  end
  pts
end

num_records = `wc -l files.csv`.split[0].to_i
puts "Parsing #{num_records} files"

f = File.open("fred.btsf","w")
# Header
f.print [1, 4*4, 2*4, num_records].pack("L*".freeze)

record_num = 0
start_time = Time.now
CSV.foreach("files.csv", col_sep: ';') do |row|
  file = "data/#{row[0].rstrip.gsub('\\','/')}"
  name = row[1].rstrip
  points = get_points(file)

  f.print [points.length, name.length].pack("L*".freeze)
  f.print name
  points.each do |a|
    f.print a.pack("le".freeze)
  end

  record_num += 1
  if record_num % 100 == 0
    cur_mins = (Time.now - start_time) / 60.0
    frac = record_num/num_records.to_f
    eta_mins = cur_mins / frac
    puts "#{record_num}/#{num_records} (#{(frac*100).round(3)}%) at #{cur_mins.round(2)}m ETA #{eta_mins.round(2)}m"
  end
end
