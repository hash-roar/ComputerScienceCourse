select distinct ShipName from "Order" where shipname like "%-%" 
select distinct ShipName ,substr(ShipName,1,instr(ShipName,"-")-1) from "Order" where shipname like "%-%"  order by ShipName;
