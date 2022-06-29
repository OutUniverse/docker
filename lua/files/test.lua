co = coroutine.create(
    function()
        for i = 1, 10 do
            print(i)

            if i == 3 then
                print(coroutine.status(co))
                print(coroutine.running())
            end
            coroutine.yield()
        end
    end
)

print(coroutine.status(co))

coroutine.resume(co)
coroutine.resume(co)
coroutine.resume(co)
coroutine.resume(co)


print(coroutine.status(co))
print(coroutine.running())