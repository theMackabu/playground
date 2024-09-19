function events.tick()
	 if data.time >= data.frameCount then
		  data.time = 0
		  data.nextBlink = world.getTime() + math.random(15, 130)
	 end

	 if data.sparkTime >= data.sparkCount then
		  data.sparkTime = 0
		  data.nextSpark = world.getTime() + math.random(5, 10) --Setting to make sparks (Random Generates between two numbers)
	 end

	 local sparkUVs = data.sparkTime / data.sparkCount
	 local spriteUVs = data.time / data.frameCount
	 local h = models.HenryModel.Main.Head.pixel_art_animations.neutral_blinking
	 local s = models.HenryModel.Spark

	 h["Neutral Blinking"]:setUV(0, spriteUVs)
	 h["Eye Shinies"]:setUV(0, spriteUVs)
	 if s and s["SparkCube"] then
		  s["SparkCube"]:setUV(0, sparkUVs * data.nextSpark)
	 end

	 
	 if data.nextBlink < world.getTime() then
		  data.time = data.time + 2
	 end
	 
	 if data.nextSpark < world.getTime() then
		  data.sparkTime = data.sparkTime + 2
	 end

	 h["Eye Shinies"]:setSecondaryColor(
		  math.map(world.getLightLevel(player:getPos()), 0, 0.5, 0, 0.02)
	 )
	  
	 h["ButtonEyes"]:setSecondaryColor(
		  math.map(world.getLightLevel(player:getPos()), 0, 0.5, 0, 0.02)
	 )
	 if (spriteUVs ~= 0) then
		  models.HenryModel.Main.Head.pixel_art_animations.neutral_blinking.ButtonEyes:setVisible(false)
	 else
		  models.HenryModel.Main.Head.pixel_art_animations.neutral_blinking.ButtonEyes:setVisible(true)
	 end
end